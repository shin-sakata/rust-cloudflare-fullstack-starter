use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::ui::Button;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="flex-1 flex flex-col items-center justify-center gap-4 py-20 px-6 relative z-10">
            <p class="text-[120px] font-black opacity-[0.05] leading-none m-0 bg-gradient-to-br from-accent-blue via-accent-purple to-accent-pink bg-clip-text text-transparent">"404"</p>
            <p class="text-sm text-text-secondary">"This page could not be found."</p>
            <A href="/" attr:class="no-underline">
                <Button>"Go Home"</Button>
            </A>
        </div>
    }
}
