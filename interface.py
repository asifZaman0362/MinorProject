# section mail-server

@post
def send(req):
    mail = req.body.mail
    database.save_to_outbox(mail)
    return status(await mta.send_mail(mail, target))

@get
def fetch(timestamp):
    if database.validate_mail(timestamp):
        return await database.fetch()
    else return None

# end section

# section transfer agent

@socket_method
def send(req):
    mail = parse_request(req).mail
    target = parse_request(req).target
    target_ip = database.resolve_target(target)
    if target_ip:
        return status(await socket::send(target_ip, req))
    else:
        if parent_mta:
            parent_mta.forward_mail(target) # or whatever the hell the status code for failure is

# subsection database

@database
def resolve_target(target):
    results = await inner_db_api.query(f'select ip from map_table where name = {target}')
    if results.empty():
        if root_resolver:
            return await root_resolver.resolve_target(target)
        else:
            return None
    else:
        return results
