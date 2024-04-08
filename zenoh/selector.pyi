class Selector:
    """A selector is the combination of a Key Expression, which defines the set of keys that are relevant to an operation, and a set of parameters with a few intendend uses:
    specifying arguments to a queryable, allowing the passing of Remote Procedure Call parameters
    filtering by value,
    filtering by metadata, such as the timestamp of a value,
    specifying arguments to zenoh when using the REST API.
    When in string form, selectors look a lot like a URI, with similar semantics:
    the key_expr before the first ? must be a valid key expression.
    the parameters after the first ? should be encoded like the query section of a URL:
    parameters are separated by &,
    the parameter name and value are separated by the first =,
    in the absence of =, the parameter value is considered to be the empty string,
    both name and value should use percent-encoding to escape characters,
    defining a value for the same parameter name twice is considered undefined behavior, with the encouraged behaviour being to reject operations when a duplicate parameter is detected.
    Zenoh intends to standardize the usage of a set of parameter names. To avoid conflicting with RPC parameters, the Zenoh team has settled on reserving the set of parameter names that start with non-alphanumeric characters.
    The full specification for selectors is available here , it includes standardized parameters.
    Queryable implementers are encouraged to prefer these standardized parameter names when implementing their associated features, and to prefix their own parameter names to avoid having conflicting parameter names with other queryables.
    Here are the currently standardized parameters for Zenoh (check the specification page for the exhaustive list):
    _time: used to express interest in only values dated within a certain time range, values for this parameter must be readable by the Zenoh Time DSL for the value to be considered valid.
    [unstable] _anyke: used in queries to express interest in replies coming from any key expression. By default, only replies whose key expression match query's key expression are accepted. _anyke disables the query-reply key expression matching check.
    """

    def __new__(cls, arg: IntoSelector): ...

IntoSelector = Selector | str
