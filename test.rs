use std::net::url::Url;
use url_from_str = std::net::url::from_str;
use std::cell::Cell;
use util::{DataStream, VoidPtrLike};
use values::*;
use color::{Color, rgb};
use select::{SelectCtx, SelectHandler};
use stylesheet::Stylesheet;
use computed::ComputedStyle;

fn test_url() -> Url {
    result::unwrap(url_from_str("http://foo.com"))
}

fn style_stream(style: &str) -> DataStream {
    let style = Cell(style.to_str());
    |move style| if !style.is_empty() {
        Some(str::to_bytes(style.take()))
    } else {
        None
    }
}

enum TestNode = @NodeData;

struct NodeData {
    name: ~str,
    children: ~[TestNode],
    mut parent: Option<TestNode>
}

impl TestNode: VoidPtrLike {
    static fn from_void_ptr(node: *libc::c_void) -> TestNode {
        assert node.is_not_null();
        TestNode(unsafe {
            let box = cast::reinterpret_cast(&node);
            cast::bump_box_refcount(box);
            box
        })
    }

    fn to_void_ptr(&self) -> *libc::c_void {
        unsafe { cast::reinterpret_cast(&(*self)) }
    }
}

struct TestHandler {
    bogus: int
}

impl TestHandler {
    static fn new() -> TestHandler {
        TestHandler {
            bogus: 0
        }
    }
}

impl TestHandler: SelectHandler<TestNode> {
    fn node_name(node: &TestNode) -> ~str { copy (*node).name }
    fn named_parent_node(node: &TestNode, name: &str) -> Option<TestNode> {
        match (**node).parent {
            Some(parent) => {
                if name == (**parent).name {
                    Some(parent)
                } else {
                    None
                }
            }
            None => None
        }
    }
    fn parent_node(node: &TestNode) -> Option<TestNode> { (**node).parent }
}

fn single_div_test(style: &str, f: &fn(&ComputedStyle)) {
    let sheet = Stylesheet::new(test_url(), style_stream(style));
    let mut select_ctx = SelectCtx::new();
    let handler = &TestHandler::new();
    select_ctx.append_sheet(move sheet);
    let dom = &TestNode(@NodeData {
        name: ~"div",
        children: ~[],
        parent: None
    });
    let style = select_ctx.select_style(dom, handler);
    let computed = style.computed_style();
    f(&computed);
}

#[test]
fn test_background_color_simple() {
    let style = "div { background-color: #123456; }";
    do single_div_test(style) |computed| {
        let color = computed.background_color();
        assert color == Specified(rgb(0x12, 0x34, 0x56));
    }
}

#[test]
fn test_border_top_width_px() {
    let style = "div { border-top-width: 10px; }";
    do single_div_test(style) |computed| {
        let width = computed.border_top_width();
        assert width == Specified(CSSBorderWidthLength(Px(10.0)));
    }
}

#[test]
fn test_border_right_width_px() {
    let style = "div { border-right-width: 10px; }";
    do single_div_test(style) |computed| {
        let width = computed.border_right_width();
        assert width == Specified(CSSBorderWidthLength(Px(10.0)));
    }
}

#[test]
fn test_border_bottom_width_px() {
    let style = "div { border-bottom-width: 10px; }";
    do single_div_test(style) |computed| {
        let width = computed.border_bottom_width();
        assert width == Specified(CSSBorderWidthLength(Px(10.0)));
    }
}

#[test]
fn test_border_left_width_px() {
    let style = "div { border-left-width: 10px; }";
    do single_div_test(style) |computed| {
        let width = computed.border_left_width();
        assert width == Specified(CSSBorderWidthLength(Px(10.0)));
    }
}

#[test]
fn test_border_width_px() {
    let style = "div { border-width: 10px; }";
    do single_div_test(style) |computed| {
        let width = computed.border_top_width();
        assert width == Specified(CSSBorderWidthLength(Px(10.0)));
        let width = computed.border_right_width();
        assert width == Specified(CSSBorderWidthLength(Px(10.0)));
        let width = computed.border_bottom_width();
        assert width == Specified(CSSBorderWidthLength(Px(10.0)));
        let width = computed.border_left_width();
        assert width == Specified(CSSBorderWidthLength(Px(10.0)));
    }
}

#[test]
fn test_border_color() {
    let style = "div {\
                 border-top-color: red;\
                 border-right-color: green;\
                 border-bottom-color: blue;\
                 border-left-color: yellow;\
                 }";
    do single_div_test(style) |computed| {
        let top_color = computed.border_top_color();
        let right_color = computed.border_right_color();
        let bottom_color = computed.border_bottom_color();
        let left_color = computed.border_left_color();
        assert top_color == Specified(rgb(255, 0, 0));
        assert right_color == Specified(rgb(0, 128, 0));
        assert bottom_color == Specified(rgb(0, 0, 255));
        assert left_color == Specified(rgb(255, 255, 0));
    }
}

#[test]
fn test_border_color_shorthand() {
    let style = "div {\
                 border-color: red;\
                 }";
    do single_div_test(style) |computed| {
        let top_color = computed.border_top_color();
        let right_color = computed.border_right_color();
        let bottom_color = computed.border_bottom_color();
        let left_color = computed.border_left_color();
        assert top_color == Specified(rgb(255, 0, 0));
        assert right_color == Specified(rgb(255, 0, 0));
        assert bottom_color == Specified(rgb(255, 0, 0));
        assert left_color == Specified(rgb(255, 0, 0));
    }
}

#[test]
fn test_margin() {
    let style = "div {\
                 margin-top: 10px;\
                 margin-right: 20px;\
                 margin-bottom: 30px;\
                 margin-left: auto;\
                 }";
    do single_div_test(style) |computed| {
        assert computed.margin_top() == Specified(CSSMarginLength(Px(10.0)));
        assert computed.margin_right() == Specified(CSSMarginLength(Px(20.0)));
        assert computed.margin_bottom() == Specified(CSSMarginLength(Px(30.0)));
        assert computed.margin_left() == Specified(CSSMarginAuto);
    }
}



fn child_test(style: &str, f: &fn(&ComputedStyle)) {
    let sheet = Stylesheet::new(test_url(), style_stream(style));
    let mut select_ctx = SelectCtx::new();
    let handler = &TestHandler::new();
    select_ctx.append_sheet(move sheet);
    let child = TestNode(@NodeData {
        name: ~"span",
        children: ~[],
        parent: None
    });
    let parent = TestNode(@NodeData {
        name: ~"div",
        children: ~[child],
        parent: None
    });
    child.parent = Some(parent);
    let style = select_ctx.select_style(&child, handler);
    let computed = style.computed_style();
    f(&computed);
}

#[test]
fn test_child() {
    let style = "div > span { border-left-width: 10px; }";
    do child_test(style) |computed| {
        let width = computed.border_left_width();
        assert width == Specified(CSSBorderWidthLength(Px(10.0)));
    }
}

#[test]
fn test_not_child() {
    let style = "div > not_span { border-left-width: 10px; }";
    do child_test(style) |computed| {
        let width = computed.border_left_width();
        assert width != Specified(CSSBorderWidthLength(Px(10.0)));
    }
}

#[test]
#[ignore]
fn test_descendant() {
    let style = "div span { border-left-width: 10px; }";
    do child_test(style) |computed| {
        let width = computed.border_left_width();
        assert width == Specified(CSSBorderWidthLength(Px(10.0)));
    }
}