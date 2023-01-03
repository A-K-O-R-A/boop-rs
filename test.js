/**
 * @typedef Metadata
 * @type {object}
 * @property {string} name - the name of this plugin
 */

/** @returns {Metadata} */
function metadata() {
  return {
    name: "Remove Newlines",
  };
}

/**
 * Run this plugin
 * @param {string} state
 * @returns {string}
 */
function run(state) {
  return state.replace(/(\n|  )/g, "");
}
