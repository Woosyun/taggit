import { invoke } from "@tauri-apps/api/tauri";
import { Note } from "./types";
import { search } from "./main";

let noteOptionPopover: HTMLDivElement;

export function initNoteOption() {
    noteOptionPopover = document.querySelector("#note-option-popover") as HTMLDivElement;
    if (!noteOptionPopover) {
        console.error('missing note-option-popover');
        return;
    }
}

export function getNoteOptionPopoverTrigger(note: Note): HTMLButtonElement {
    const button = document.createElement("button");
    button.classList.add('note-option-popover-trigger');
    button.innerText = ':';
    button.addEventListener('click', (e: any) => {
        e.stopPropagation();
        setNoteOption(note);
    });
    button.popoverTargetElement = noteOptionPopover;

    return button;
}

async function setNoteOption(note: Note) {
    noteOptionPopover.innerHTML = '';

    const title = document.createElement("h3");
    title.innerText = note.title;
    noteOptionPopover.appendChild(title);

    //render tags
    const tagSet = await invoke('upetch_tag_note', { noteId: note.id, tags: [], isUpdate: false }) as Set<string>;
    const tagbar = document.createElement("div");
    tagbar.innerText = 'Tags: ';
    tagbar.classList.add('tagbar');
    const tags = Array.from(tagSet);
    tags.forEach((tag: any) => {
        console.log('tag: ', tag);
        
        const tagElement = document.createElement("span");
        tagElement.classList.add('badge');
        tagElement.innerText = tag.name;
        tagElement.id = tag.id;
        tagbar.appendChild(tagElement);
    });
    noteOptionPopover.appendChild(tagbar);

    
    const deleteButton = document.createElement("button");
    deleteButton.innerText = 'Delete';
    deleteButton.addEventListener('click', async () => {
        console.log('deleting note: ', note.id);
        await invoke('delete_note', { noteId: note.id });
        await search();
    });
    noteOptionPopover.appendChild(deleteButton);
}