use leptos::prelude::*;

use thaw::{
    Button, ButtonAppearance, DrawerBody, DrawerHeader, DrawerHeaderTitle, DrawerHeaderTitleAction,
    DrawerModalType, DrawerPosition, OverlayDrawer,
};

#[component]
// pub fn BottomModal(open: RwSignal<bool>, title: RwSignal<String>, children: Children) -> impl IntoView {
pub fn BottomModal(
    open: RwSignal<bool>,
    title: RwSignal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <OverlayDrawer open position=DrawerPosition::Bottom modal_type=DrawerModalType::NonModal>
            <DrawerHeader>
                <DrawerHeaderTitle>
                    <DrawerHeaderTitleAction slot>
                        <Button
                            appearance=ButtonAppearance::Subtle
                            on_click=move |_| open.set(false)
                        >
                            "x"
                        </Button>
                    </DrawerHeaderTitleAction>
                            <h6>{ move ||title.get() }</h6>
                </DrawerHeaderTitle>
            </DrawerHeader>
            <DrawerBody>
                <div class="row">
                    <div class="col-10 offset-2">

                    { children() }

                    </div>
                </div>
            </DrawerBody>
        </OverlayDrawer>
    }
}
