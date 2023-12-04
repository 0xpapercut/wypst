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
