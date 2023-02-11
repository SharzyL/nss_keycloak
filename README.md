# NSS Module for Keycloak

Work in progress

## Keycloak Configuration

In your realm (e.g. `school`), create a client (e.g. `nss`). Create a user which is used to sending API queries to keycloak. Assign it with the following roles:

- (realm-management) query-groups
- (realm-management) query-users
- (realm-management) view-users

Create a group. Put the users that you want nss to list to this group. Each user in this group should be given the following attribtues:
- `github_id` is the GitHub user id used to retrive public key (not used now).
- `uid` is the uid and gid that nss with assign to.

## Module Setup

Put the config file in `/etc/kcnss.toml`. The path to this file can be specified via `KEYCLOAK_NSS_CONF` environment variable. But specifying via env may bring security issues, so this feature will be audited later.

```toml
username = "nss"
password = "your-password"
base_url = "https://keycloak.example.com"
realm = "school"
client_id = "nss"
group_id = "ed35e3db-1145-5541-a08f-e250adf058ab"
default_shell = "/bin/bash"
```

Edit `/etc/nsswitch.conf`, e.g.
```
passwd: keycloak files systemd
group: keycloak files [SUCCESS=merge] systemd
shadow: keycloak files systemd
```

Compile the module with `nix build` or `cargo build --release`. Put the compiled `libnss_keycloak.so` in `/lib/libnss_keycloak.so.2`.

Now the nss module should be setup. Try it with `getent passwd`.

