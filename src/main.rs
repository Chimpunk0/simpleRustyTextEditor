mod editor;
use editor::Editor;

fn main() {
    let editor = Editor::default();
    // no need to specify the struct reference
    // when called on a specific instance, rust automatically
    // dereferences the instance to call the method
    // editor.run() is equivalent to (&editor).run()
    editor.run();
}
