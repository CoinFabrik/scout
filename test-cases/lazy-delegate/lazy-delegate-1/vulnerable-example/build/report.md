![Banner Scout report](https://www.example.com/banner.png)
# Scout Report - 2024-03-20 09:52:32

## Summary
 - [Lazy Delegate](lazy-delegate) (2 results) (Critical)
 - [Unsafe Delegate Call](unsafe-delegate-call) (1 results) (Critical)
 - [Check Ink! version](check-ink!-version) (1 results) (Enhancement)
 - [Unrestricted Transfer From](unrestricted-transfer-from) (2 results) (Critical)

## Known Bugs

### Lazy Delegate

**Impact:** Critical

**Description:** Delegate call with non-lazy, non-mapping storage

**More about:** [here]()

<table style="width: 100%; table-layout: fixed;"><thead><tr><th style="width: 20%;">ID</th><th style="width: 60%;">Detection</th><th style="width: 20%;">Status</th></tr></thead><tbody>
<tr><td>0</td><td><a href="link-to-github">lib.rs:49:18 - 49:26</a></td><td><ul><li>- [ ] False Positive </li><li>- [ ] Acknowledged</li><li>- [ ] Resolved</li></ul></td></tr>
<tr><td>1</td><td><a href="link-to-github">lib.rs:12:5 - 15:6</a></td><td><ul><li>- [ ] False Positive </li><li>- [ ] Acknowledged</li><li>- [ ] Resolved</li></ul></td></tr>
</tbody></table>

</tbody></table>


## Authorization 

### Unsafe Delegate Call

**Impact:** Critical

**Description:** Passing arguments to the target of a delegate call is not safe, as it allows the caller to set a malicious hash as the target.

**More about:** [here]()

<table style="width: 100%; table-layout: fixed;"><thead><tr><th style="width: 20%;">ID</th><th style="width: 60%;">Detection</th><th style="width: 20%;">Status</th></tr></thead><tbody>
<tr><td>3</td><td><a href="link-to-github">lib.rs:48:23 - 49:34</a></td><td><ul><li>- [ ] False Positive </li><li>- [ ] Acknowledged</li><li>- [ ] Resolved</li></ul></td></tr>
</tbody></table>

</tbody></table>


## Best practices

### Check Ink! version

**Impact:** Enhancement

**Description:** Use the latest version of ink!

**More about:** [here](Using a older version of ink! can be dangerous, as it may have bugs or security issues. Use the latest version available.    )

<table style="width: 100%; table-layout: fixed;"><thead><tr><th style="width: 20%;">ID</th><th style="width: 60%;">Detection</th><th style="width: 20%;">Status</th></tr></thead><tbody>
<tr><td>2</td><td><a href="link-to-github">Cargo.toml</a></td><td><ul><li>- [ ] False Positive </li><li>- [ ] Acknowledged</li><li>- [ ] Resolved</li></ul></td></tr>
</tbody></table>

</tbody></table>


## Validations and error handling

### Unrestricted Transfer From

**Impact:** Critical

**Description:** This argument comes from a user-supplied argument

**More about:** [here]()

### Zero or Test Address

**Impact:** Medium

**Description:** Not checking for a zero-address could lead to a locked contract

**More about:** [here]()

<table style="width: 100%; table-layout: fixed;"><thead><tr><th style="width: 20%;">ID</th><th style="width: 60%;">Detection</th><th style="width: 20%;">Status</th></tr></thead><tbody>
<tr><td>4</td><td><a href="link-to-github">lib.rs:52:26 - 52:45</a></td><td><ul><li>- [ ] False Positive </li><li>- [ ] Acknowledged</li><li>- [ ] Resolved</li></ul></td></tr>
<tr><td>5</td><td><a href="link-to-github">lib.rs:42:13 - 42:33</a></td><td><ul><li>- [ ] False Positive </li><li>- [ ] Acknowledged</li><li>- [ ] Resolved</li></ul></td></tr>
</tbody></table>

</tbody></table>

