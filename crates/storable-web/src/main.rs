use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");
static FAVICON: Asset = asset!("/assets/favicon.ico");
static TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: TAILWIND_CSS }
        document::Link { rel: "icon", href: FAVICON }

        div { class: "p-6",
            h1 { class: "text-2xl font-bold mb-4", "Storable Cloud" }

            table { class: "w-full border-collapse border",
                thead {
                    tr {
                        th { class: "border p-2", "Name" }
                        th { class: "border p-2", "Size" }
                        th { class: "border p-2", "Type" }
                    }
                }
                tbody {
                    tr {
                        td { class: "border p-2", "example.txt" }
                        td { class: "border p-2", "1234 bytes" }
                        td { class: "border p-2", "text/plain" }
                    }
                    tr {
                        td { class: "border p-2", "image.png" }
                        td { class: "border p-2", "4567 bytes" }
                        td { class: "border p-2", "image/png" }
                    }
                }
            }
        }
    }
}
