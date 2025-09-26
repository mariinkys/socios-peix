// SPDX-License-Identifier: GPL-3.0-only

use leptos::prelude::*;

use crate::core::api::users::Logout;

#[component]
pub fn NavbarComponent(dark_mode: RwSignal<bool>) -> impl IntoView {
    let logout = ServerAction::<Logout>::new();
    Effect::new(move |_| {
        if let Some(val) = logout.value().get() {
            match val {
                Ok(_) => {
                    if let Some(win) = web_sys::window() {
                        win.location().reload().unwrap();
                    }
                }
                Err(err) => {
                    eprintln!("{err}");
                }
            }
        }
    });

    view! {
        <div class="navbar bg-base-100 shadow-sm">
            <div class="navbar-start">
                <div class="dropdown">
                <div tabindex="0" role="button" class="btn btn-ghost btn-circle">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"> <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h7" /> </svg>
                </div>
                <ul
                    tabindex="0"
                    class="menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-52 p-2 shadow">
                    <li><a href="/members">"Socios"</a></li>
                    <li><a href="/interests">"Intereses"</a></li>
                    <li><a href="/management">"Gestión"</a></li>
                    <li><a href="/users">"Usuarios"</a></li>
                    <li><a href="/cupon-check">"Comprobar Cupón"</a></li>
                </ul>
                </div>
            </div>
            <div class="navbar-center">
                <a class="btn btn-ghost text-xl" href="/">"Socios Peix"</a>
            </div>
            <div class="navbar-end gap-3">
                <label class="swap swap-rotate">
                    <input
                        type="checkbox"
                        prop:checked=move || dark_mode.get()
                        on:input=move |_| dark_mode.update(|v| *v = !*v)
                    />
                    <svg class="swap-on fill-current w-10 h-10" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z"/></svg>
                    <svg class="swap-off fill-current w-10 h-10" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z"/></svg>
                </label>
                <button
                    on:click=move |_| { logout.dispatch(Logout {  }); }
                >
                    <svg class="fill-current w-9 h-9" viewBox="0 0 32 32" version="1.1" xmlns="http://www.w3.org/2000/svg">
                        <path
                            d="M25.229,14.5l-16.003,0c-0.828,-0 -1.5,0.672 -1.5,1.5c-0,0.828 0.672,1.5 1.5,1.5l16.038,0l-3.114,3.114c-0.585,0.585 -0.585,1.536 0,2.121c0.586,0.586 1.536,0.586 2.122,0c-0,0 2.567,-2.567 4.242,-4.242c1.367,-1.367 1.367,-3.583 0,-4.95l-4.242,-4.243c-0.586,-0.585 -1.536,-0.585 -2.122,0c-0.585,0.586 -0.585,1.536 0,2.122l3.079,3.078Z" />
                        <path
                            d="M20,24l-0,-4.5l-10.774,0c-1.932,-0 -3.5,-1.568 -3.5,-3.5c-0,-1.932 1.568,-3.5 3.5,-3.5l10.774,0l-0,-4.5c-0,-2.761 -2.239,-5 -5,-5c-2.166,-0 -4.834,0 -7,-0c-1.326,-0 -2.598,0.527 -3.536,1.464c-0.937,0.938 -1.464,2.21 -1.464,3.536c-0,4.439 -0,11.561 0,16c-0,1.326 0.527,2.598 1.464,3.536c0.938,0.937 2.21,1.464 3.536,1.464c2.166,0 4.834,0 7,0c1.326,0 2.598,-0.527 3.536,-1.464c0.937,-0.938 1.464,-2.21 1.464,-3.536Z" />
                    </svg>
                </button>
            </div>
        </div>
    }
}
