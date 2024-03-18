/// Generate user `.cabal`, taking `--enable-nix` option into account
pub(crate) fn generate(name: &str, module: &str, version: &str, enable_nix: bool) -> String {
    let build_type = if enable_nix {
        "
build-type:         Simple"
    } else {
        "
-- This let us hook Cabal steps to Setup.lhs script.
build-type:         Custom
custom-setup
    setup-depends:  Cabal, base, directory, process"
    };

    let lib_name = name.replace('-', "_"); // In library generated by Cargo, '-' is replaced by '_'
    let package_name = name.replace('_', "-"); // Cabal does not expect '_' for packages names

    let extra = if enable_nix {
        format!(
            "
    -- `haskell.nix` tell GHC linker where to find the `libNAME.a` by setting
    -- automatically `extra-lib-dirs`:
    -- https://input-output-hk.github.io/haskell.nix/tutorials/pkg-map.html
    extra-libraries:  {lib_name}

    -- Cross-compilation to target `x86_64-w64-mingw32-cc` thrown a lot of
    -- `undefined reference to 'X'` errors during linking stage ...
  if os(windows)
    extra-libraries: userenv ws2_32 bcrypt
    -- Here is a mapping between library names and missing symbols:
    -- `bcrypt`  -> `BCryptGenRandom`
    -- `userenv` -> `GetUserProfileDirectoryW`
    -- `ws2_32`  -> `freeaddrinfo getaddrinfo WSASend WSARecv WSASocketW`"
        )
    } else {
        format!(
            "
    -- Libraries that are bundled with the package.
    extra-bundled-libraries: {lib_name}"
        )
    };

    format!(
        "cabal-version:      2.4
-- The cabal-version field refers to the version of the .cabal specification,
-- and can be different from the cabal-install (the tool) version and the
-- Cabal (the library) version you are using. As such, the Cabal (the library)
-- version used must be equal or greater than the version stated in this field.
-- Starting from the specification version 2.2, the cabal-version field must be
-- the first thing in the cabal file.

-- Initial package description generated by 'cabal init'. For further
-- documentation, see: http://haskell.org/cabal/users-guide/
--
-- The name of the package.
name:               {package_name}

-- The package version.
-- See the Haskell package versioning policy (PVP) for standards
-- guiding when and how versions should be incremented.
-- https://pvp.haskell.org
-- PVP summary:     +-+------- breaking API changes
--                  | | +----- non-breaking API additions
--                  | | | +--- code changes with no API change
version:            {version}

-- A short (one-line) description of the package.
-- synopsis:

-- A longer description of the package.
-- description:

-- The license under which the package is released.
-- license:

-- The package author(s).
-- author:

-- An email address to which users can send suggestions, bug reports, and
-- patches.
-- maintainer:

-- A copyright notice.
-- copyright:

{build_type}

-- Extra doc files to be distributed with the package, such as a CHANGELOG or a
-- README.
-- extra-doc-files:

-- Extra source files to be distributed with the package, such as examples, or
-- a tutorial module.
-- extra-source-files:
--
-- FIXME: It's still unclear to me what would be the best strategy to let users
-- publish packages generated by `cargo-cabal` on Hackage. While it is pretty
-- hazardous to put Rust code in sdist archive (because that would require that
-- the library end-user have a Rust developer environment on this machine and
-- that wouldn't play well with cross-compilation), is it a good idea to
-- package generated platform-dependent library as source?

common warnings
    ghc-options: -Wall

library
    -- Import common warning flags.
    import:           warnings

    -- Modules exported by the library.
    exposed-modules:  {module}

    -- Modules included in this library but not exported.
    -- other-modules:

    -- LANGUAGE extensions used by modules in this package.
    -- other-extensions:

    -- Other library packages from which modules are imported.
    build-depends:    base

    -- Directories containing source files.
    hs-source-dirs:   src

    -- Base language which the package is written in.
    default-language: Haskell2010

{extra}

-- This file was generated by `cargo-cabal`, but please don't hesitate to edit it!

-- We would rather rely on `cabal init --non-interactive` to generate this file
-- but there is no CLI arg to set `build-type: Custom` on which it sadly
-- currently have to rely on."
    )
}
