use chrono::{TimeZone, Utc};
use chrono_humanize::HumanTime;
use yew::prelude::*;

use crate::{
    components::{delete_account::DeleteAccount, update_password::UpdatePassword},
    hooks::use_profile,
};

#[function_component(Content)]
fn content() -> HtmlResult {
    let user_profile = use_profile()?;

    let now = Utc::now();

    let created_at_parsed = Utc.from_local_datetime(&user_profile.created_at).unwrap();
    let member_since = format!("Member since {}", HumanTime::from(created_at_parsed - now));

    let updated_at_parsed = Utc.from_local_datetime(&user_profile.updated_at).unwrap();
    let last_update_since = format!(
        "Last update since {}",
        HumanTime::from(updated_at_parsed - now)
    );

    Ok(html! {
        <div class="flex flex-col gap-4">
            <div class="text-center text-slate-100">
                <p class="text-lg font-semibold ">{user_profile.email}</p>
                <div class="mt-4 text-sm text-slate-600">
                    <p>{member_since}</p>
                    <p>{last_update_since}</p>
                </div>
            </div>
            <UpdatePassword />
            <DeleteAccount />
        </div>
    })
}

#[function_component(Profile)]
pub fn profile() -> Html {
    let fallback = html! {
        <div class="text-xl text-slate-100">{ "Loading..." }</div>
    };

    html! {
        <div class="flex flex-col items-center max-w-md mx-auto">
            <header class="mb-6 text-2xl font-bold text-orange-500">
                <h1>{ "Profile" }</h1>
            </header>
            <Suspense {fallback}>
                <Content />
            </Suspense>
        </div>
    }
}
