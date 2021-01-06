use gdnative::prelude::*;
use gdnative::api::{EditorPlugin, Resource, Script, Texture, ImageTexture, Image};
use gdnative::prelude::Null;
use usvg::{ViewBox, AspectRatio, Rect, NodeKind};
use std::ops::DerefMut;

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
    svg_path: String,
    image_texture: Option<Ref<ImageTexture>>,
}

#[methods]
impl SVGPoly {
    fn new(_owner: TRef<Node2D>) -> Self {
        SVGPoly {
            svg_path: String::new(),
            image_texture: None,
        }
    }

    #[export]
    fn _enter_tree(&mut self, _owner: TRef<Node2D>) {
        if self.image_texture.is_none() {
            let usvg_tree = usvg::Tree::from_file(&self.svg_path, &usvg::Options::default()).unwrap();

            {
                let mut usvg_root_node = usvg_tree.root();
                let mut ref_mut = usvg_root_node.borrow_mut();
                let der_mut = ref_mut.deref_mut();
                let svg_node = match der_mut {
                    NodeKind::Svg(ref mut svg) => svg,
                    _ => unreachable!(),
                };
                svg_node.view_box = ViewBox {
                    aspect: AspectRatio::default(),
                    rect: Rect::new(50., 50., 200., 200.).unwrap(),
                };
            }

            let rendered_img = resvg::render(&usvg_tree, usvg::FitTo::Zoom(2.), None).unwrap();

            let img = Image::new();
            img.create_from_data(rendered_img.width().into(), rendered_img.height().into(), false,
                                 Image::FORMAT_RGBA8, TypedArray::from_slice(rendered_img.data()));

            let image_texture = ImageTexture::new();
            image_texture.create_from_image(img, 7);

            // As I'm giving the texture to the engine (to draw it), I need it to be shared
            // This is a thread safety measure (as the engine could possibly keep a ref and modify the content concurrently)
            // To operate on it however, I will have to use unsafe code and assume I'm the only one to actually have the ref (which should be the case)
            self.image_texture = Some(image_texture.into_shared());
        }
    }

    #[export]
    fn _exit_tree(&mut self, _owner: TRef<Node2D>) {
        if self.image_texture.is_some() {
            self.image_texture = None
        }
    }

    #[export]
    fn _draw(&self, owner: TRef<Node2D>) {
        if self.image_texture.is_some() {
            owner.draw_texture(self.image_texture.as_ref().unwrap(), Vector2::new(20., 20.),
                               Color::rgba(1., 1., 1., 1.), Null::null());
        }
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
