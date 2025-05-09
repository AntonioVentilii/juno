use crate::hooks::js::runtime::runner::{execute_hook, make_loader_code};
use crate::hooks::js::runtime::types::{JsHook, OnJsHook};
use crate::hooks::js::types::hooks::JsHookContext;
use junobuild_satellite::OnDeleteFilteredDocsContext;
use rquickjs::{Ctx, Error as JsError};

const ON_FUNCTION: &str = "onDeleteFilteredDocs";

pub struct OnDeleteFilteredDocs;

impl JsHook for OnDeleteFilteredDocs {
    fn get_loader_code(&self) -> String {
        make_loader_code(
            ON_FUNCTION,
            "__juno_satellite_on_delete_filtered_docs_loader",
        )
    }
}

impl OnJsHook<OnDeleteFilteredDocsContext> for OnDeleteFilteredDocs {
    async fn execute<'js>(
        &self,
        ctx: &Ctx<'js>,
        context: OnDeleteFilteredDocsContext,
    ) -> Result<(), JsError> {
        let js_context = JsHookContext::from_on_delete_filtered_docs(ctx, context)?;
        execute_hook(ctx, js_context, ON_FUNCTION).await
    }
}
