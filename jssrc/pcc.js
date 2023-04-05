export function pearsonCorrelationCoefficient(x, y) {
  const n = x.length;
  let sum_x = 0;
  let sum_y = 0;
  let square_sum_x = 0;
  let square_sum_y = 0;
  let product_sum = 0;

  for (let i = 0; i < n; i++) {
    sum_x += x[i];
    sum_y += y[i];
    square_sum_x += x[i] * x[i];
    square_sum_y += y[i] * y[i];
    product_sum += x[i] * y[i];
  }

  const numerator = n * product_sum - sum_x * sum_y;
  const denominator = Math.sqrt((n * square_sum_x - sum_x * sum_x) * (n * square_sum_y - sum_y * sum_y));

  if (denominator === 0) {
    return 0;
  } else {
    return numerator / denominator;
  }
}

