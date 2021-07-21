/** @file connection.h
    @brief Client connection.

    This file includes the type definition for a connection structure,
   representing a client connected to an OpenTok session, along with several
   function declarations related to client connections.
*/
#ifndef CONNECTION_H
#define CONNECTION_H

#include "config.h"
#include "base.h"

OTC_BEGIN_DECL

/** Type definition for a structure representing a client connected to an
   OpenTok session.
 */
typedef struct otc_connection otc_connection;

/**
    Returns the unique identifier for this connection.

    @param connection The client connection.
    @return The unique identifier for this connection.
 */
OTC_DECL(const char*) otc_connection_get_id(const otc_connection* connection);

/**
    Returns the timestamp for when the client connected to the OpenTok session.

    @param connection The client connection.
    @return The time when the client connected to the OpenTok session.
 */
OTC_DECL(int64_t)
otc_connection_get_creation_time(const otc_connection* connection);

/**
    Returns the data associated with the connection. You set this data when you
   create the token used for by the client to connect to the OpenTok session.
   See the <a
   href="https://tokbox.com/developer/guides/create-token/#connection-data">
    Token Creation Overview</a> developer guide.

    @param connection The client connection.
    @return The connection data.
 */
OTC_DECL(const char*) otc_connection_get_data(const otc_connection* connection);

/**
    Gets the session ID for the session the connection is connected to.

    @param connection The client connection.
    @return The session ID for the session.
 */
OTC_DECL(const char*)
otc_connection_get_session_id(const otc_connection* connection);

/**
    Makes a copy of this connection.

    @param connection The client connection to be copied
    @return A copy of the connection. It is set to null if there is an error.
 */
OTC_DECL(otc_connection*) otc_connection_copy(const otc_connection* connection);

/**
    Releases resources associated with the connection.

    @param connection The client connection to release.
    @return Return value indicating either error or success.
 */
OTC_DECL(otc_status) otc_connection_delete(otc_connection* connection);

OTC_END_DECL

#endif  // CONNECTION_H
