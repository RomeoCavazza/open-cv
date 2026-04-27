export function countGs(str) {
    return str.split('').filter(char => char === 'G').length;
}