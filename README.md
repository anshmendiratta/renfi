# renfi
A personal project to avoid manually renaming files. Used in tandem with [autosort](github.com/anshmendiratta/autosortassignments).

## Usage
It looks for the names of folders listed inside the directory stored in the environment variable `ASSIGNMENTS_DIR`. Set this to whatever folders you wish to organize files into.
Once set, to use the TUI, run `cargo r -- {file}` where {file} is what you wish to rename.

E.g., `cargo r -- college/exam_3.pdf` with the assignments folder (e.g., `college/`) containing folders named `math/`, `chem/`, and `psych/` would give you to the options: 
- `math_exam_3.pdf`
- `chem_exam_3.pdf`
- `psych_exam_3.pdf` 

and store the resulting file in `college/`.

Of course, for easier use, store the resulting binary in a folder included in `$PATH`. This does mean that you would need to recompile the binary every time there is a push to the repo, but if it already works, this is far more convenient to have than needing to be in your clone of the repo.

## Compatibility
Aside from varying file systems on different operating systems, none of the internals of the script should be OS-specific. If you do find a bug, please open an issue/PR.

- Windows, MacOS, Linux: Complete.

## Maintenance
While the repository is for personal use, it is made public for those who find a utility like this useful. Since this is a small script, PRs and issues will be tended to.
