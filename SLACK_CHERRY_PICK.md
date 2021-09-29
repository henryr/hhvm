Maintains a running list of github.com/facebook/hhvm commits cherry-picked into this release.

| upstream SHA | commit summary |
|--------------|----------------|
| none | 4602404b1f2a0812465eef15e0cf9697607c3bf2 is a Slack patch to enable us to compile without USE_JSONC while still retaining permissive json_decode behaviors |
| none | c890b8c388ccc762e4a186099da02a8e2a45b8c5 is a Slack patch to make json_decode return darrays or varrays by default, not just darrays |
| [9ee508af6c1baf5e64999ae1a3dc4cd9af27b014](https://github.com/facebook/hhvm/commit/9ee508af6c1baf5e64999ae1a3dc4cd9af27b014) | Backported fixes to boost dependency detection code |
| [6e609158e681fad05056dad6e4674da46897f39b](https://github.com/facebook/hhvm/commit/6e609158e681fad05056dad6e4674da46897f39b) | Backported fix to a race condition in the build process that leads to build flakiness |
| [5be1acd5313e0121bf3a14ea48cd5e3c63d4019b](https://github.com/facebook/hhvm/commit/5be1acd5313e0121bf3a14ea48cd5e3c63d4019b) | Fixes for timelib which otherwise break our timezone libraries |
| [daa159471acfcdf912582e707bdbea236b9a66f2](https://github.com/facebook/hhvm/commit/daa159471acfcdf912582e707bdbea236b9a66f2) | More timelib fixes |
| [60284bed4dd8d850918e98c975851068304a82db](https://github.com/facebook/hhvm/commit/60284bed4dd8d850918e98c975851068304a82db) | One of 4 commits necessary to fix VSCode breakpoints in lambdas |
| [2c2d6126b5c6f4cb3d9c11fc6979801deeefcaf5](https://github.com/facebook/hhvm/commit/2c2d6126b5c6f4cb3d9c11fc6979801deeefcaf5) | One of 4 commits necessary to fix VSCode breakpoints in lambdas |
| [ef87738de820131c819ded391106ce0364441ef5](https://github.com/facebook/hhvm/commit/ef87738de820131c819ded391106ce0364441ef5) | One of 4 commits necessary to fix VSCode breakpoints in lambdas |
| [0f0ab3bf2937c3ac413a9ae9640878c86a0dad02](https://github.com/facebook/hhvm/commit/0f0ab3bf2937c3ac413a9ae9640878c86a0dad02) | One of 4 commits necessary to fix VSCode breakpoints in lambdas |