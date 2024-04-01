mod facade;

pub trait Command<Ctx = ()> {
    type Ok;
    type Err;

    fn execute(&self, ctx: &Ctx) -> Result<Self::Ok, Self::Err>;
}

pub trait Gateway<Cmd, Ctx = ()>
where
    Cmd: Command<Ctx>,
{
    type Ok;
    type Err;

    fn send(&self, cmd: Cmd) -> Result<Self::Ok, Self::Err>;
}

pub trait ExtractFrom<Val> {}
