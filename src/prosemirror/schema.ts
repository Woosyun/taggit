import { Schema } from "prosemirror-model";

const schema = new Schema({
    nodes: {
        doc: { content: 'heading block*' },
        paragraph: {
            content: 'inline*',
            group: 'block',
            toDOM() { return ["p", 0]; },
            parseDOM: [{ tag: "p" }]
        },
        blockquote: {
            content: 'block+',
            group: 'block',
            toDOM() { return ["blockquote", 0]; },
            parseDOM: [{ tag: "blockquote" }],
        },
        horizontal_rule: {
            group: 'block',
            toDOM() { return ["hr"]; },
            parseDOM: [{ tag: "hr" }],
        },
        text: {
            group: "inline", // Ensures text is treated as inline content
        },
        heading: {
            content: "inline*",
            group: "block",
            toDOM(node: any) { return ["h" + node.attrs.level, 0]; },
            defining: true,
            parseDOM: [
                { tag: "h1", attrs: { level: 1 } },
                { tag: "h2", attrs: { level: 2 } },
                { tag: "h3", attrs: { level: 3 } },
                { tag: "h4", attrs: { level: 4 } },
                { tag: "h5", attrs: { level: 5 } },
                { tag: "h6", attrs: { level: 6 } }
            ],
            attrs: { level: { default: 1 } }
        },
        ordered_list: {
            content: "list_item+",
            group: "block",
            attrs: { order: { default: 1, validate: "number" } },
            parseDOM: [{
                tag: "ol", getAttrs(dom: HTMLElement) {
                    return { order: dom.hasAttribute("start") ? +dom.getAttribute("start")! : 1 }
                }
            }],
            toDOM(node) { return node.attrs.order == 1 ? ["ol", 0] : ["ol", { start: node.attrs.order }, 0] }
        },
        bullet_list: {
            content: "list_item+",
            group: "block",
            parseDOM: [{ tag: "ul" }],
            toDOM() { return ["ul", 0] }
        },
        list_item: {
            content: "paragraph block*",
            parseDOM: [{ tag: "li" }],
            toDOM() { return ["li", 0] },
            defining: true
        },
        code_block: {
            content: "text*",
            marks: "",
            group: "block",
            code: true,
            defining: true,
            parseDOM: [{ tag: "pre", preserveWhitespace: "full" }],
            toDOM() { return ["pre", ["code", 0]] }
        }
    },
    marks: {
        strong: {},
        em: {}
    }
});

export default schema;