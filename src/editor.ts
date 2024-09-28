import { EditorView } from "prosemirror-view";
import { EditorState } from "prosemirror-state";
import 'prosemirror-view/style/prosemirror.css'

import schema from './prosemirror/schema.ts';
import getCommandBindings from "./prosemirror/commands.ts";
import { keymap } from "./prosemirror/official/keymap.ts";
import { buildInputRules } from './prosemirror/official/input_rules.ts';
import './prosemirror/style.css';
import { roughByteSizeOfObject } from "./utils_client.ts";

const plugins = [
    keymap(getCommandBindings(schema)),
    buildInputRules(schema),
]

const state = EditorState.create({
    schema,
    plugins
});

declare global {
    interface Window {
        view: EditorView,
    }
}

window.view = new EditorView(document.getElementById('note-container'), {
    state,
    editable: () => true,
    dispatchTransaction(tr) {
        const state = window.view.state.apply(tr);

        const content = JSON.stringify(state.toJSON());
        // console.log('stringified content: ', content);

        const json = JSON.parse(content);
        // console.log('parsed content: ', json);

        // const newState = EditorState.fromJSON(
        //     { schema, plugins },
        //     content,
        // );

        // console.log(roughByteSizeOfObject(content) + 'bytes content made');
        // console.log(roughByteSizeOfObject(json) + 'bytes object made');

        
        // localStorage.setItem('editorContent', JSON.stringify(content));
        window.view.updateState(state);
    },
});