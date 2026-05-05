function appendChildren(parent, children) {
    const queue = Array.isArray(children) ? children.flat(Infinity) : [children];
    queue.forEach((child) => {
        if (child == null || child === false) return;
        if (child instanceof Node) {
            parent.appendChild(child);
        } else {
            parent.appendChild(document.createTextNode(String(child)));
        }
    });
}

function applyProps(node, props = {}) {
    Object.entries(props).forEach(([key, value]) => {
        if (value == null || value === false) return;
        if (key === 'style') {
            if (typeof value === 'string') node.style.cssText = value;
            else Object.assign(node.style, value);
            return;
        }
        if (key === 'dataset') {
            Object.assign(node.dataset, value);
            return;
        }
        if (key === 'attrs') {
            Object.entries(value).forEach(([attrKey, attrValue]) => {
                if (attrValue != null && attrValue !== false) node.setAttribute(attrKey, String(attrValue));
            });
            return;
        }
        if (key === 'text') {
            node.textContent = String(value);
            return;
        }
        const isSvg = node instanceof SVGElement;
        
        if (key.startsWith('on') && typeof value === 'function') {
            node[key] = value;
            return;
        }

        if (!isSvg && key in node && key !== 'class') {
            node[key] = value;
            return;
        }
        node.setAttribute(key, String(value));
    });
}

export function clear(node) {
    node.replaceChildren();
    return node;
}

export function text(value) {
    return document.createTextNode(String(value ?? ''));
}

export function el(tagName, props = {}, children = []) {
    const node = document.createElement(tagName);
    applyProps(node, props);
    appendChildren(node, children);
    return node;
}

export function svg(tagName, props = {}, children = []) {
    const node = document.createElementNS('http://www.w3.org/2000/svg', tagName);
    applyProps(node, props);
    appendChildren(node, children);
    return node;
}

export function safeClick(id, handler) {
    const el = document.getElementById(id);
    if (el) el.onclick = handler;
}