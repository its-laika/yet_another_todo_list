let todos = [];

function loadTodos() {
    fetch(`${getOrigin()}/todos/open`)
        .then(response => response.json())
        .then(t => todos = t)
        .then(_ => Handlebars.templates.table({ todos }))
        .then(html => document.getElementById('table-container').innerHTML = html);
}

function addTodo(event) {
    if (event) {
        event.preventDefault();
    }

    const body = {
        text: event.target[0].value,
        done: false,
    };

    fetch(
        `${getOrigin()}/todos`,
        {
            method: 'POST',
            body: JSON.stringify(body),
            headers: {
                'Content-Type': 'application/json'
            }
        }
    )
        .then(response => response.json())
        .then(t => todos.push(t))
        .then(_ => Handlebars.templates.table({ todos }))
        .then(html => document.getElementById('table-container').innerHTML = html);
}

function updateTodo(todoId, done) {
    const body = {
        text: todos.find(t => t.id === todoId).text,
        done
    };

    fetch(
        `${getOrigin()}/todos/${todoId}`,
        {
            method: 'PUT',
            body: JSON.stringify(body),
            headers: {
                'Content-Type': 'application/json'
            }
        }
    )
        .then(_ => todos.forEach(t => {
            if (t.id === todoId) {
                t.done = done;
            }
        }))
        .then(_ => Handlebars.templates.table({ todos }))
        .then(html => document.getElementById('table-container').innerHTML = html);
}

function deleteTodo(todoId) {
    fetch(`${getOrigin()}/todos/${todoId}`, { method: 'DELETE' })
        .then(_ => todos = todos.filter(t => t.id !== todoId))
        .then(_ => Handlebars.templates.table({ todos }))
        .then(html => document.getElementById('table-container').innerHTML = html);
}

/* Convenience function that allows us to globally use another origin for e.g. development */
function getOrigin() {
    return window.location;
}

document.querySelector('form').addEventListener('submit', addTodo);
loadTodos();