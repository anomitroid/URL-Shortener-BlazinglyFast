new ClipboardJS('.short-url', {
    text: function(trigger) {
        return window.location.origin + trigger.getAttribute('href');
    }
}).on('success', function(e) {
    alert('URL copied to clipboard!');
    e.clearSelection();
});

function enlargeQR(img) {
    const modal = document.getElementById('qrModal');
    const enlargedImg = document.getElementById('enlargedQR');
    const container = document.querySelector('.container');
    container.classList.toggle('modal-open');
    enlargedImg.src = img.src;
    modal.style.display = 'block';
}

document.addEventListener('DOMContentLoaded', () => {
    const modal = document.getElementById('qrModal');
    const closeBtn = document.querySelector('.close');
    const container = document.querySelector('.container');
    
    closeBtn.addEventListener('click', () => {
        modal.style.display = 'none';
        container.classList.toggle('modal-open');
    });

    window.addEventListener('click', (event) => {
        if (event.target === modal) {
            modal.style.display = 'none';
            container.classList.toggle('modal-open');
        }
    });
});