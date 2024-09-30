import {
    baseKeymap,
    chainCommands,
    createParagraphNear,
    liftEmptyBlock,
    newlineInCode,
    splitBlock
} from './official/commands'
import {
    wrapInList,
    splitListItem,
    splitListItemKeepMarks,
    liftListItem,
    sinkListItem
} from './official/schema_list'
import { Schema } from 'prosemirror-model'
import { Command } from 'prosemirror-state'
import { invoke } from '@tauri-apps/api/tauri'
import { Note } from '../types'
import { getTags } from '../tags'

export default function getCommandBindings(schema: Schema, note: Note | null): {[key: string]: Command} {
    const listItemType = schema.nodes.list_item;

    // console.log('listItemType is ', listItemType);
    
    let keymap: {[key: string]: Command} = {
        ...baseKeymap,
        "Enter": listItemType
            ? chainCommands(newlineInCode, splitListItem(listItemType), createParagraphNear, liftEmptyBlock, splitBlock)
            : baseKeymap["Enter"],
        "ctrl-s": () => {
            storeNote(note)
            return true;
        },
    }

    return keymap;
}

async function storeNote(note: Note | null) {
    console.log('(storeNote) note: ', note);

    const json = window.view.state.toJSON();

    try {
        const title = json.doc.content[0].content[0].text;
        const content = JSON.stringify(json);
        const note_id = await invoke('upsert_note', { noteId: note ? note.id : 0, title, content, isUpdate: note !== null }) as number;

        console.log('(storeNote) note id: ', note_id);
        
        const tags = await invoke('upetch_tag_note', { noteId: note_id, tags: getTags(), isUpdate: true });
        console.log('(storeNote) returned tags: ', tags);
    } catch (error: any) {
        console.error("(storeNote) erro: ", error);
    }
}