# le_version_info

Small utility to simply record at which point in time which Boulder version was deployed.

A live version of this can be found on my site: https://germancoding.com/boulder-version-history/

To build this program, you'll need:

1. Rust + Cargo (https://rustup.rs/)
2. If compiling under Linux: OpenSSL + OpenSSL development headers (see [this](https://github.com/seanmonstar/reqwest#requirements))
3. build-essential/native compiler (probably?)

Then, simply run <code>cargo build --release</code> to build the program (Cargo builds to the <code>./target</code> directory).

## Example usage

<code>./le_version_info file.json https://acme-v02.api.letsencrypt.org/build</code>

Note that file.json must already exist and must contain a valid JSON array (empty is fine).

<code>
echo "[]" > file.json
</code>

Each invocation tries to contact the given URL, treating the output as a build. The JSON file is then updated accordingly. 
You'll probably need to run this periodically to be useful.
