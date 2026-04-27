function range(start, end, step) {
  if (step === undefined) {
    step = 1;
  }

  var result = [];
  if (step > 0) {
    for (var i = start; i <= end; i += step) {
      result.push(i);
    }
  } else {
    for (var i = start; i >= end; i += step) {
      result.push(i);
    }
  }
  return result;
}

module.exports = { range };