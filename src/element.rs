#![allow(non_camel_case_types)]

use crate::{attribute::AnyAttribute, batch::Batch, InNamespace, NodeId};

use self::sealed::Sealed;

mod sealed {
    use crate::{Element, InNamespace};

    pub trait Sealed {}

    impl Sealed for Element {}
    impl<'a> Sealed for &'a str {}
    impl<'a> Sealed for InNamespace<'a, Element> {}
    impl<'a, 'b> Sealed for InNamespace<'a, &'b str> {}
}

pub enum AnyElement<'a, 'b> {
    Element(Element),
    InNamespace(InNamespace<'a, Element>),
    Str(&'a str),
    InNamespaceStr(InNamespace<'a, &'b str>),
}

impl AnyElement<'_, '_> {
    pub fn encode(&self, v: &mut Batch) {
        match self {
            AnyElement::Element(a) => a.encode(v),
            AnyElement::InNamespace(a) => a.encode(v),
            AnyElement::Str(a) => a.encode(v),
            AnyElement::InNamespaceStr(a) => a.encode(v),
        }
    }
}

/// Anything that can be turned into an element name
pub trait IntoElement<'a, 'b>: Sealed {
    fn encode(&self, v: &mut Batch);

    fn any_element(self) -> AnyElement<'a, 'b>;
}

impl<'a, 'b> Element {
    pub const fn any_element_const(self) -> AnyElement<'a, 'b> {
        AnyElement::Element(self)
    }
}

impl<'a, 'b> IntoElement<'a, 'b> for Element {
    #[inline(always)]
    fn encode(&self, v: &mut Batch) {
        v.msg.push(*self as u8);
    }

    fn any_element(self) -> AnyElement<'a, 'b> {
        AnyElement::Element(self)
    }
}

impl<'a, 'b> InNamespace<'a, Element> {
    pub const fn any_element_const(self) -> AnyElement<'a, 'b> {
        AnyElement::InNamespace(self)
    }
}

impl<'a, 'b> IntoElement<'a, 'b> for InNamespace<'a, Element> {
    fn encode(&self, v: &mut Batch) {
        v.msg.push(255);
        v.msg.push(self.0 as u8);
        v.encode_str(self.1);
    }

    fn any_element(self) -> AnyElement<'a, 'b> {
        AnyElement::InNamespace(self)
    }
}

impl<'a, 'b> IntoElement<'a, 'b> for &'a str {
    fn encode(&self, v: &mut Batch) {
        v.msg.push(254);
        v.encode_str(*self);
    }

    fn any_element(self) -> AnyElement<'a, 'b> {
        AnyElement::Str(self)
    }
}

impl<'a, 'b> IntoElement<'a, 'b> for InNamespace<'a, &'b str> {
    fn encode(&self, v: &mut Batch) {
        v.msg.push(253);
        v.encode_str(self.0);
        v.encode_str(self.1);
    }

    fn any_element(self) -> AnyElement<'a, 'b> {
        AnyElement::InNamespaceStr(self)
    }
}

impl<'a, 'b> InNamespace<'a, &'b str> {
    pub const fn any_element_const(self) -> AnyElement<'a, 'b> {
        AnyElement::InNamespaceStr(self)
    }
}

/// A builder for a element with an id, kind, attributes, and children
pub struct ElementBuilder<'a> {
    id: Option<NodeId>,
    kind: AnyElement<'a, 'a>,
    attrs: &'a [(AnyAttribute<'a, 'a>, &'a str)],
    children: &'a [ElementBuilder<'a>],
}

impl<'a> ElementBuilder<'a> {
    pub const fn new(kind: AnyElement<'a, 'a>) -> Self {
        Self {
            id: None,
            kind,
            attrs: &[],
            children: &[],
        }
    }

    pub const fn id(mut self, id: NodeId) -> Self {
        self.id = Some(id);
        self
    }

    pub const fn attrs(mut self, attrs: &'a [(AnyAttribute<'a, 'a>, &'a str)]) -> Self {
        self.attrs = attrs;
        self
    }

    pub const fn children(mut self, children: &'a [ElementBuilder<'a>]) -> Self {
        self.children = children;
        self
    }

    pub(crate) fn encode(&self, v: &mut Batch) {
        v.encode_optional_id_with_byte_bool(self.id);
        self.kind.encode(v);
        // these are packed together so they can be read as a u16
        v.msg.push(self.attrs.len() as u8);
        v.msg.push(self.children.len() as u8);
        for (attr, value) in self.attrs {
            attr.encode_u8_discriminant(v);
            v.encode_str(*value);
        }
        for child in self.children {
            child.encode(v);
        }
    }
}

/// All built-in elements
#[allow(unused)]
#[derive(Copy, Clone)]
pub enum Element {
    a,
    abbr,
    acronym,
    address,
    applet,
    area,
    article,
    aside,
    audio,
    b,
    base,
    bdi,
    bdo,
    bgsound,
    big,
    blink,
    blockquote,
    body,
    br,
    button,
    canvas,
    caption,
    center,
    cite,
    code,
    col,
    colgroup,
    content,
    data,
    datalist,
    dd,
    del,
    details,
    dfn,
    dialog,
    dir,
    div,
    dl,
    dt,
    em,
    embed,
    fieldset,
    figcaption,
    figure,
    font,
    footer,
    form,
    frame,
    frameset,
    h1,
    head,
    header,
    hgroup,
    hr,
    html,
    i,
    iframe,
    image,
    img,
    input,
    ins,
    kbd,
    keygen,
    label,
    legend,
    li,
    link,
    main,
    map,
    mark,
    marquee,
    menu,
    menuitem,
    meta,
    meter,
    nav,
    nobr,
    noembed,
    noframes,
    noscript,
    object,
    ol,
    optgroup,
    option,
    output,
    p,
    param,
    picture,
    plaintext,
    portal,
    pre,
    progress,
    q,
    rb,
    rp,
    rt,
    rtc,
    ruby,
    s,
    samp,
    script,
    section,
    select,
    shadow,
    slot,
    small,
    source,
    spacer,
    span,
    strike,
    strong,
    style,
    sub,
    summary,
    sup,
    table,
    tbody,
    td,
    template,
    textarea,
    tfoot,
    th,
    thead,
    time,
    title,
    tr,
    track,
    tt,
    u,
    ul,
    var,
    video,
    wbr,
    xmp,
}
