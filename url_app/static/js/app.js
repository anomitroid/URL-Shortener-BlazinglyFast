new ClipboardJS('.short-url', {
    text: function(trigger) {
        return window.location.origin + trigger.getAttribute('href');
    }
}).on('success', function(e) {
    alert('URL copied to clipboard!');
    e.clearSelection();
});
