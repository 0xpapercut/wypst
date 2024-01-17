export function deleteFields(obj, fields) {
    if (Array.isArray(obj)) {
        for (let i = 0; i < obj.length; i++) {
            deleteFields(obj[i], fields);
        }
    } else if (typeof obj === 'object' && obj !== null) {
        for (let field of fields) {
            delete obj[field];
        }
        for (let key in obj) {
            if (obj.hasOwnProperty(key)) {
                deleteFields(obj[key], fields);
            }
        }
    }
}

export function deleteIfFieldEquals(obj, value) {
    if (typeof obj === 'object' && obj !== null) {
        for (let key in obj) {
            if (!obj.hasOwnProperty(key))
                continue;
            if (obj[key] === value) {
                delete obj[key];
            } else {
                deleteIfFieldEquals(obj[key], value);
            }
        }
    }
}

export function diff(obj1, obj2) {
    const result = {};
    for (const key in obj1) {
        if (!obj2.hasOwnProperty(key)) {
            result[key] = obj1[key];
        } else if (typeof obj1[key] === 'object' && typeof obj2[key] === 'object') {
            const value = diff(obj1[key], obj2[key]);
            if (Object.keys(value).length !== 0) {
                result[key] = value;
            }
        } else if (obj1[key] !== obj2[key]) {
            result[key] = obj1[key];
        }
    }
    return result;
}
