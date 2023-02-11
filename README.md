# NSS module using keycloak

Work in progress

## Usage

Put the config file in `/etc/kcnss.toml` (which can be specified via `KEYCLOAK_NSS_CONF` environment variable). Note that specifying via env may bring security issues, to be audited later.

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

Put compiled `libnss_keycloak.so` in `/lib/libnss_keycloak.so.2`.

