# Cayenne Low Power Payload for Rust

This crate is a port of the [Cayenne Low Power Payload (LPP)](https://docs.mydevices.com/docs/lorawan/cayenne-lpp) API
to the Rust programming language. It provides an easy way to send data over LPWAN networks such as LoRaWAN. Cayenne LPP
is compliant with payload size restrictions, which can be lowered down to 11 bytes and allows the device to send
multiple sensor data at one time. Its focus lies on embedded systems, so it uses the "no_std" attribute to only link the
core-crate instead of the std-crate.

Additionally, it is also possible to send different sensor data in different frames. To do this, the channel value
of the data can be used.

The payload is compatible to the integrated payload formatter of the [The Things Network (TTN)](https://www.thethingsnetwork.org)
console. It can be enabled for the download or the uplink of a device. More information about the Cayenne LPP payload
formatter of the TTN can be found [here](https://www.thethingsindustries.com/docs/integrations/payload-formatters/cayenne/).

The original C++ version of Cayenne LPP can be found [here](https://github.com/myDevicesIoT/CayenneLPP).

## Example

The following example will show how to add two data types, one digital input value and one temperature value, to the
Cayenne LPP payload. Since there is no memory allocator available ("no_std"), the API needs a buffer, that will be
filled by it.  
To create a buffer with the exact size of the payload, use the size constants of the different data types, that are
provided by the API. To add two digital inputs, just multiply the size variable by 2 and use this size for the buffer
initialization. The buffer needs to be mutable since it will be modified by the API. Note that it is not necessary to
exactly define the buffer size, it just needs to be greater or equal to the size of the data types that are added to it.
Therefore, it is possible to reuse the buffer with different data types. The CayenneLPP::reset() function can be used
to reset the structure.  
After the buffer initialization, just create a new instance of the CayenneLPP API and add the data types to the payload
with the available API functions.  
After adding all the values, a slice to the buffer can be retrieved by calling the CayenneLPP::payload_slice() function.
The slice can be then used to send the data via an external API.

```rust
fn main() {
    // create the buffer for a digital input and a temperature data type
    let mut buffer: [u8; LPP_DIGITAL_INPUT_SIZE + LPP_TEMPERATURE_SIZE] = [0; LPP_DIGITAL_INPUT_SIZE + LPP_TEMPERATURE_SIZE];
    
    // create a new instance of CayenneLPP struct and initialize it with the buffer
    let mut lpp = CayenneLPP::new(&mut buffer);

    // add a digital input with the value 0x55
    lpp.add_digital_input(3, 0x55).unwrap();
    
    // add a temperature value of 12.3°C 
    lpp.add_temperature(5, 12.3).unwrap();

    // retrieve the payload slice to be able to send the payload via an external API
    let cayenne_lpp_payload = lpp.payload_slice();
    
    // ... send the data via an external API
    
    // reset the API and start from the beginning of the buffer 
    lpp.reset();

    // add new values...
    lpp.add_digital_input(3, 0xAA).unwrap();
    lpp.add_temperature(5, 32.1).unwrap();
}
```

## Future development

The API in its current state should be pretty stable to use, but it might be extended for a more detailed error response
when adding new values to the data structure in the future.  
However, if you have any remarks or want to add some functionality feel free to start a discussion or send a PR, but do
not forget to add unit tests and / or integration tests, if you want that it gets merged into the repo.

# License

The crate is released under the MIT license. See [LICENSE](./LICENSE) for more information.