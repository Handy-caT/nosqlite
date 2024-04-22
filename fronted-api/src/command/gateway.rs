use std::fmt::Debug;

use crate::{
    api::Api,
    command::{Command, Execute, Extract, Gateway},
};
use crate::api::Context;

impl<Cmd, Context, const NODE_SIZE: u8> Gateway<Cmd, Context> for Api<NODE_SIZE>
where
    Self: Execute<Cmd, Context> + Extract<Context>,
    <Self as Execute<Cmd, Context>>::Err: Debug,
    Cmd: Command,
{
    type Ok = <Self as Execute<Cmd, Context>>::Ok;
    type Err = <Self as Execute<Cmd, Context>>::Err;

    fn send(
        &mut self,
        cmd: Cmd,
    ) -> Result<
        <Self as Gateway<Cmd, Context>>::Ok,
        <Self as Gateway<Cmd, Context>>::Err,
    > {
        let ctx = self.extract();
        <Self as Execute<Cmd, Context>>::execute(cmd, ctx)
    }
}

impl<const NODE_SIZE: u8> Extract<Context> for Api<NODE_SIZE> {
    fn extract(&mut self) -> &mut Context {
        &mut self.context
    }
}
