function addTextBox() {
    const text = new fabric.Textbox('Click to edit (Supports Markdown)', {
        left: 100,
        top: 100,
        width: 200,
        fontSize: 16
    });

    text.on('editing:exited', function() {
        const markdown = this.text;
        const html = marked.parse(markdown);
        this.set('text', html);
        canvas.renderAll();
    });

    canvas.add(text);
    canvas.renderAll();
}