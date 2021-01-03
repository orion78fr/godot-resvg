use gdnative::prelude::*;
use gdnative::api::{EditorPlugin, Resource, Script, Texture};

#[derive(NativeClass)]
#[inherit(EditorPlugin)]
struct GodotResvgEditorPlugin;

#[methods]
impl GodotResvgEditorPlugin {
    fn new(_owner: TRef<EditorPlugin>) -> Self {
        GodotResvgEditorPlugin
    }

    #[export]
    fn _enter_tree(&self, owner: TRef<EditorPlugin>) {
        let script = unsafe {
            load::<Script>("res://addons/godot_resvg/svg_poly.gdns", "Script").unwrap()
        };
        let texture = unsafe {
            load::<Texture>("res://addons/godot_resvg/svg.png", "Texture").unwrap()
        };
        owner.add_custom_type("SVG Polygon", "Node2D", script, texture);
    }

    #[export]
    fn _exit_tree(&self, owner: TRef<EditorPlugin>) {
        owner.remove_custom_type("SVG Polygon");
    }
}

#[derive(NativeClass)]
#[inherit(Node2D)]
struct SVGPoly {
    #[property(path = "SVG Path")]
    svg_path: String
}

#[methods]
impl SVGPoly {
    fn new(_owner: TRef<Node2D>) -> Self {
        SVGPoly {
            svg_path: String::new()
        }
    }

    #[export]
    fn _enter_tree(&self, _owner: TRef<Node2D>) {}

    #[export]
    fn _process(&self, _owner: TRef<Node2D>, _delta: f64) {
        //godot_print!("Path is {}", self.svg_path)
    }

    #[export]
    fn _draw(&self, owner: TRef<Node2D>) {
        owner.draw_circle(Vector2::new(25., 25.), 10., Color::rgb(1., 0., 0.));
        owner.draw_circle(Vector2::new(45., 25.), 10., Color::rgb(1., 0., 0.));
        owner.draw_line(Vector2::new(35., 25.), Vector2::new(35., 55.),
                        Color::rgb(1., 0., 0.), 10., true)
    }
}

unsafe fn load<T>(path: &str, hint: &str) -> Option<Ref<T, Shared>>
    where
        T: GodotObject<RefKind=RefCounted> + SubClass<Resource>,
{
    let resource = ResourceLoader::godot_singleton().load(path, hint, false)?;
    let resource = resource.assume_safe().claim();
    resource.cast::<T>()
}

fn init(handle: InitHandle) {
    handle.add_tool_class::<GodotResvgEditorPlugin>();
    handle.add_tool_class::<SVGPoly>();
}

godot_init!(init);
