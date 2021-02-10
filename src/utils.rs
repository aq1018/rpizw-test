pub fn convert_nb_error<E>(r: Result<u8, nb::Error<E>>) -> Result<Option<u8>, E> {
    match r {
        Ok(v) => Ok(Some(v)),
        Err(e) => match e {
            nb::Error::WouldBlock => Ok(None),
            nb::Error::Other(e) => Err(e),
        },
    }
}
