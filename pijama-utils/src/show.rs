pub trait Show<Ctx>: Sized {
    fn show(&self, ctx: &Ctx, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;

    fn wrap<'ctx>(&self, ctx: &'ctx Ctx) -> ShowRef<'ctx, Ctx, &Self> {
        ShowRef { inner: self, ctx }
    }
}

pub struct ShowRef<'ctx, Ctx, S: Show<Ctx>> {
    inner: S,
    ctx: &'ctx Ctx,
}

impl<Ctx, S: Show<Ctx>> Show<Ctx> for &S {
    fn show(&self, ctx: &Ctx, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (*self).show(ctx, f)
    }
}

impl<'ctx, Ctx, S: Show<Ctx>> std::fmt::Display for ShowRef<'ctx, Ctx, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.show(self.ctx, f)
    }
}
