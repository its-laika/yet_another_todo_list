Handlebars.registerHelper('tryLink', function (value) {
    if (!value) {
        return value;
    }

    if (typeof value !== 'string') {
        return value;
    }

    try {
        if ((new URL(value)).protocol !== 'https:') {
            return value;
        }
        return new Handlebars.SafeString(`<a href="${value}" target="_blank">${value}</a>`);
    } catch (_) {
        return value;
    }
});

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

    const input = event.target[0]
    if (!input) {
        throw new Error('Could not find input element');
    }

    input.disabled = true;

    const text = input.value;
    if (!text) {
        return;
    }

    const body = {
        text,
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
        .then(html => document.getElementById('table-container').innerHTML = html)
        .then(_ => {
            input.disabled = false;
            input.value = '';
        });
}

function updateTodo(todoId, done) {
    const text = todos.find(t => t.id === todoId).text;
    if (!text) {
        return;
    }

    const body = {
        text,
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
        .then(html => document.getElementById('table-container').innerHTML = html)
        .then(_ => new Promise(resolve => setTimeout(resolve, 3000)))
        .then(_ => loadTodos());
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