import { initEditor, renderEditor } from './editor';
import './styles.css';
import { invoke } from '@tauri-apps/api/tauri';
import { Note } from './types';
import { addTag, getTags, initTagbar, renderTagbar } from './tagbar';
import { getNoteOptionPopoverTrigger, initNoteOption } from './note_option';

let itemContainer: any;
window.addEventListener('DOMContentLoaded', () => {
    initEditor();
    initTagbar();
    initNoteOption();

    itemContainer = document.querySelector('#item-container');
    if (!itemContainer) {
        console.error('missing tagbar or note-container or item-container');
        return;
    }

    // add note creation button
    document.querySelector('#search-form')?.addEventListener('submit', handleSubmit);
    
    //TODO: add condition to check if a note already on editing
    document.querySelector('#new-note-button')?.addEventListener('click', () => renderEditor(null));
});

async function handleSubmit(e: any) {
    e?.preventDefault();

    const searchInput = document.querySelector('#search-input') as HTMLInputElement;
    if (!searchInput) return;
    //render tagbar only when tags are updated
    if (addTag(searchInput.value)) {
        renderTagbar();
    }

    await search();
    searchInput.value = '';
}

export async function search() {
    console.log('searching... tags: ', getTags());
    
    const notes = await invoke('search_by_tags', { tags: getTags() }) as Note[];
    console.log('search result: ', notes);

    itemContainer.innerHTML = '';
    notes.forEach(note => {
        const card = document.createElement("div");
        card.classList.add('note-card');
        card.innerText = note.title;
        card.addEventListener('click', async () => {
            console.log("fetching note: ", note.id);
            invoke('fetch_note', { noteId: note.id })
                .then(note => {
                    console.log('fetched note: ', note);
                    renderEditor(note as Note);
                });
        });

        //add option button ":"
        card.appendChild(getNoteOptionPopoverTrigger(note));
        itemContainer.appendChild(card);
    });
}
