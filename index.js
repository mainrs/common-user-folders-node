const { loadBinding } = require('@node-rs/helper')

/**
 * __dirname means load native addon from current dir
 * 'common-user-folders' is the name of native addon
 * the second arguments was decided by `napi.name` field in `package.json`
 * the third arguments was decided by `name` field in `package.json`
 * `loadBinding` helper will load `common-user-folders.[PLATFORM].node` from `__dirname` first
 * If failed to load addon, it will fallback to load from `@sirwindfield/common-user-folders-[PLATFORM]`
 */
module.exports = loadBinding(__dirname, 'common-user-folders', '@sirwindfield/common-user-folders')
