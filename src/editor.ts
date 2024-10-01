import { EditorView } from "prosemirror-view";
import { EditorState } from "prosemirror-state";
import 'prosemirror-view/style/prosemirror.css'
import schema from './prosemirror/schema.ts';
import getCommandBindings from "./prosemirror/commands.ts";
import { keymap } from "./prosemirror/official/keymap.ts";
import { buildInputRules } from './prosemirror/official/input_rules.ts';
import './prosemirror/style.css';

import { Note } from "./types.ts";

let noteContainer: HTMLDivElement;
export function initEditor() {
    noteContainer = document.querySelector('#note-container') as HTMLDivElement;
    if (!noteContainer) {
        console.error('missing note-container');
        return;
    }
}

declare global {
    interface Window {
        view: EditorView,
    }
}

export const renderEditor = async (note: Note | null) => {
    //fetch note if not null. Content of note is empty at first
    // let note = _note;
    // if (note !== null) {
    //     note = await invoke('fetch_note', { noteId: note.id }) as Note;
    // }
    
    const plugins = [
        keymap(getCommandBindings(schema, note)),
        buildInputRules(schema),
    ]
    
    const state = !note
        ? EditorState.create({
            schema,
            plugins
        })
        : EditorState.fromJSON(
            { schema, plugins },
            JSON.parse(note.content)
        );


    noteContainer.innerHTML = '';
    
    window.view = new EditorView(noteContainer, {
        state,
        editable: () => true,
        // dispatchTransaction(tr) {
        //     const state = window.view.state.apply(tr);
    
        //     // const content = JSON.stringify(state.toJSON());
        //     // console.log('stringified content: ', content);
    
        //     // const json = JSON.parse(content);
        //     // console.log('parsed content: ', json);
    
        //     // const newState = EditorState.fromJSON(
        //     //     { schema, plugins },
        //     //     content,
        //     // );
    
        //     // console.log(roughByteSizeOfObject(content) + 'bytes content made');
        //     // console.log(roughByteSizeOfObject(json) + 'bytes object made');
            
        //     window.view.updateState(state);
        // },
    });
}