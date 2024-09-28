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

export default function getCommandBindings(schema: Schema): {[key: string]: Command} {
    const listItemType = schema.nodes.list_item;

    // console.log('listItemType is ', listItemType);
    
    let keymap: {[key: string]: Command} = {
        ...baseKeymap,
        "Enter": listItemType
            // ? splitListItem(listItemType)
            ? chainCommands(newlineInCode, splitListItem(listItemType), createParagraphNear, liftEmptyBlock, splitBlock)
            : baseKeymap["Enter"]
    }

    return keymap;
}