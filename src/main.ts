import { renderEditor } from './editor';
import './styles.css';
import { invoke } from '@tauri-apps/api/tauri';
import { Note } from './types';
import { addTag, deleteTag, getTags } from './tags';

let tagbar: any;
let noteContainer: any;
let itemContainer: any;

window.addEventListener('DOMContentLoaded', () => {
    tagbar = document.querySelector('#tagbar');
    noteContainer = document.querySelector('#note-container');
    itemContainer = document.querySelector('#item-container');
    if (!tagbar || !noteContainer || !itemContainer) {
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

    search();
    searchInput.value = '';
}

function renderTagbar() {
    tagbar.innerHTML = '';
    getTags().forEach(tag => {
        const tagElement = document.createElement('div');
        tagElement.classList.add('badge');
        tagElement.innerHTML = tag;
        tagElement.addEventListener('click', () => {
            if (!deleteTag(tag)) return;
            renderTagbar();
            search();
        });
        tagbar.appendChild(tagElement);
    });
}

async function search() {
    console.log('searching... tags: ', getTags());
    
    const notes = await invoke('search_by_tags', { tags: getTags() }) as Note[];
    console.log('search result: ', notes);

    itemContainer.innerHTML = '';
    notes.forEach(note => {
        const item = document.createElement("p");
        item.innerText = note.title;
        item.addEventListener('click', async () => {
            //fetch note
            console.log("fetching note: ", note.id);
            invoke('fetch_note', { noteId: note.id })
                .then(note => {
                    console.log('fetched note: ', note);
                    renderEditor(note as Note);
                });
        });
        itemContainer.appendChild(item);
    });
}