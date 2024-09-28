let divider: any;
let leftPane: any;
let rightPane: any;

window.addEventListener('DOMContentLoaded', function () {
    divider = document.querySelector('#divider');
    leftPane = document.querySelector('#sidebar');
    rightPane = document.querySelector('#note-container');
    if (!divider || !leftPane || !rightPane) return;

    leftPane.style.width = "20vw";
    rightPane.style.width = `calc(100% - ${leftPane.style.width} - 5px)`;

    divider.addEventListener('mousedown', function() {
        isResizing = true;
    });
    document.addEventListener('mouseup', function() {
        isResizing = false;
    });
    document.addEventListener('mousemove', function(e) {
        if (!isResizing) return;
    
        const containerOffsetLeft = leftPane.offsetLeft;
        if (containerOffsetLeft === undefined) return;
    
        const newLeftWidth = e.clientX - containerOffsetLeft;
    
        leftPane.style.width = `${newLeftWidth}px`;
        rightPane.style.width = `calc(100% - ${newLeftWidth}px - 5px)`;

        // console.log('leftPane.style.width', leftPane.style.width);
    });
});

let isResizing = false;



