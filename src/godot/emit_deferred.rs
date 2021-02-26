use gdnative::prelude::{Node, Shared, Variant};
use gdnative::Ref;

pub trait EmitDeferred {
    fn emit_deferred(&self, signal: impl AsRef<str>, varargs: &[Variant]);
}

impl EmitDeferred for Ref<Node, Shared> {
    fn emit_deferred(&self, signal: impl AsRef<str>, varargs: &[Variant]) {
        let mut args = vec![Variant::from_str(signal)];
        args.extend_from_slice(varargs);
        // args.extend(varargs);
        unsafe {
            self.assume_safe()
                .call_deferred("emit_signal", args.as_slice());
        }
    }
}
