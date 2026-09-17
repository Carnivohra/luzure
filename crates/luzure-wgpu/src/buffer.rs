pub(crate) fn buffer_capacity(required: usize, stride: usize, limit: u64) -> Option<(usize, u64)> {
    let stride = u64::try_from(stride).ok()?;
    let maximum = usize::try_from(limit.min(usize::MAX as u64).checked_div(stride)?).ok()?;

    if required == 0 || required > maximum {
        return None;
    }

    let capacity = required.checked_next_power_of_two().unwrap_or(maximum).min(maximum);
    let size = u64::try_from(capacity).ok()?.checked_mul(stride)?;

    Some((capacity, size))
}
