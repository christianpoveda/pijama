pub trait Show<Ctx> {
    fn show(&self, ctx: &Ctx, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;

    fn wrap<'ctx, 'show>(&'show self, ctx: &'ctx Ctx) -> ShowRef<'ctx, 'show, Ctx, Self> {
        ShowRef { inner: self, ctx }
    }

    fn show_sep<'show>(slice: &'show [Self], sep: &'show str) -> ShowSeq<'show, Self>
    where
        Self: Sized,
    {
        ShowSeq { slice, sep }
    }
}

pub struct ShowRef<'ctx, 'show, Ctx, S: Show<Ctx> + 'show + ?Sized> {
    ctx: &'ctx Ctx,
    inner: &'show S,
}

impl<Ctx, S: Show<Ctx>> Show<Ctx> for &S {
    fn show(&self, ctx: &Ctx, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (*self).show(ctx, f)
    }
}

impl<'ctx, 'show, Ctx, S: Show<Ctx>> std::fmt::Display for ShowRef<'ctx, 'show, Ctx, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.show(self.ctx, f)
    }
}

pub struct ShowSeq<'show, S> {
    slice: &'show [S],
    sep: &'show str,
}

impl<'show, Ctx, S: Show<Ctx>> Show<Ctx> for ShowSeq<'show, S> {
    fn show(&self, ctx: &Ctx, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut items = self.slice.iter();

        if let Some(item) = items.next() {
            item.show(ctx, f)?;

            for item in items {
                write!(f, "{}{}", self.sep, item.wrap(ctx))?
            }
        }

        Ok(())
    }
}
