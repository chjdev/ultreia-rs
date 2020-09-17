use crate::game::{Game, Configuration};
use crate::create_game;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize, Deserializer, Serializer};

struct GameVisitor;

impl<'de> serde::de::Visitor<'de> for GameVisitor {
    type Value = *mut Game;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an unsigned integer representation of a pointer")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        Ok(u64::from(value) as *mut Game)
    }
}


pub struct GamePointer {
    pointer: *mut Game
}

impl GamePointer {
    pub fn new(configuration: Configuration) -> Self {
        unsafe {
            Self::from(create_game(configuration))
        }
    }

    pub fn from(pointer: *mut Game) -> Self {
        GamePointer { pointer }
    }

    pub fn as_mut(&self) -> Option<&mut Game> {
        unsafe {
            self.pointer.as_mut()
        }
    }

    pub fn as_ref(&self) -> Option<&Game> {
        unsafe {
            self.pointer.as_ref()
        }
    }

    pub fn with_game<R: serde::Serialize>(&self, fun: impl FnOnce(&Game) -> R) -> HttpResponse {
        if let Some(game) = self.as_ref() {
            return HttpResponse::Ok().json(fun(game));
        }
        HttpResponse::BadRequest().json("invalid game pointer")
    }

    pub fn with_game_void(&self, fun: impl FnOnce(&Game) -> ()) -> HttpResponse {
        if let Some(game) = self.as_ref() {
            fun(game);
            return HttpResponse::Ok().finish();
        }
        HttpResponse::BadRequest().json("invalid game pointer")
    }

    pub fn with_game_mut<R: serde::Serialize>(&self, fun: impl FnOnce(&mut Game) -> R) -> HttpResponse {
        if let Some(game) = self.as_mut() {
            return HttpResponse::Ok().json(fun(game));
        }
        HttpResponse::BadRequest().json("invalid game pointer")
    }
}


impl<'de> Deserialize<'de> for GamePointer {
    fn deserialize<D>(deserializer: D) -> Result<GamePointer, D::Error>
        where
            D: Deserializer<'de>,
    {
        let maybe_ptr = deserializer.deserialize_u64(GameVisitor);
        maybe_ptr.map(GamePointer::from)
    }
}

impl Serialize for GamePointer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_u64(self.pointer as u64)
    }
}
