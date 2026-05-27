Code should be as simple, concise and elegant as possible.
Make sure we can elegantly handle errors, but don't overcomplicate error handling.
NEVER ignore linting rules! fix the problems.
Always look for opportunities to strengthen the linting rules.
Consolidate domain types in one location so we have a strong domain model.
Always try and improve the type checking.
Try and avoid duplication between types.
Use immer in TypeScript where possible to avoid mutation.
Use Zustand to manage state in React.
Try to keep the state machine simple, consolidated and easy to understand.
REMOVE ALL COMMENTS! unless they are really important.
Don't add a logging facility, the console is fine.
Logging should be sufficient to diagnose problems at all levels but not overdone.
Use blank lines to make code easier to read.
Please ALWAYS! keep the README file up to date.
When reviewing a file, should it be broken up? can it be improved? is this functionality in the right location within the project?
Keep the test coverage high.
We are using vitest for unit testing.
We are using playwright for end to end testing.
Don't write unit tests for UI components.
Add data-testid to all UI components for e2e testing.
Extract logic from UI components into the lib folder so it can be unit tested.
Keep the .gitignore file up to date.
Keep the .cursorignore file up to date - it should ignore large model files and build artifacts that slow down indexing.
Please use Rust where you can, but use Python you must.
Keep up to date documentation in the docs folder.
Be rigorous in fixing warnings and errors.
Try to keep documents concise and to the point, consolidate information where possible.
Dont give me a list of options to choose, devise the best plan you can and proceed with it.
Always run training and other long running scripts using caffeinate.
Refer to this working game I made for inspiration or if you get stuck: https://github.com/rgilks/rgou-cloudflare
When deleting temporary test files, consider how they might be integrated into more permanent tests.
Code files should preferably be less than 200 lines long.
Functions should preferably be less than 20 lines long.
