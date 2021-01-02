// this config includes typescript specific settings
// and if you're not using typescript, you should remove `transform` property
module.exports = {
    transform: {
      "^.+\\.tsx?$": "ts-jest",
    },
    testRegex: "src/__tests__/.*(test|spec).(jsx?|tsx?)$",
    testPathIgnorePatterns: ["lib/", "node_modules/"],
    moduleFileExtensions: ['ts', 'tsx', 'js', 'jsx', 'json', 'node'],
    testEnvironment: "node",
    rootDir: "src",
    verbose: true,
    moduleNameMapper: {
      '^@/(.*)$': '<rootDir>/$1',
    }
  };