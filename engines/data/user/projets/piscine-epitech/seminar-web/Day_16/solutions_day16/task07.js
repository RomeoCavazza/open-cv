export function arrayFiltering(array, test) {
  let tableau = [];
  for (let element of array) {
    if (test(element) === true) {
      tableau.push(element);
    }
  }
  return tableau;
}