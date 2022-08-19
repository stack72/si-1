# Attribute Value Update

## High Level Process

- Assumption: an AttributeValue has been set somewhere
- After the value is set
- Grab the prop id, look at its parent
- If the parent has an implicit internal provider (re: you've found the first
  prop that isn't under a map or array), you stop there--we're going to do
  something with it
- For the implicit internal provider you need to create or update an attribute
  of the same level of specificity as the original attribute value that
  triggered the change
- Also need to look to see if there are values of more specificity to update
  them.
- To update: grab an AttributeView for the prop for the implicit internal
  provider, the view becomes the unprocessed and processed value for the
  internal provider
- Then, look at what attribute prototype arguements reference the implicit
  internal provider we just updated (because we know these are dependent on what
  was updated), that yields us a set of AttributePrototypes that are referenced
  by those arguments
- Once we have the prototypes, we looking for the AttributeValue that would use
  those prototypes, but the same or more specific as the original
  AttributeValue.
- Note we need to make AttributeValues in some of these slots, that is assuming
  we're not yet writing out appropriate proxy values when setting (we're likely
  doing this partially)
- At this point, we have set of AttributeValues and we can filter out values
  that are sealed proxes and attribute values that are more specific which are
  not candidates for updating.
- Now for the prototypes, we need to get the arguments for the prototypes in
  order to update the associated AttributeValue (the argument could reference an
  internal provider id, among other possibles)
- Get the AttributeValue for each internal provider is of an appropriate context
  for each of the Values we're working through. That's the resolved
  input/argument we need to create a new FuncBinding and execution to yield a
  new FuncBindingReturnValue. We update that value's fbrv and fb ids.
- The trick is to delay creating the FuncBinding only when all appropriate
  arguments have been resolved
- If we all the way back to that first impplicit internal provider for the
  original attribute value, we need to find if there is a parent prop--it's
  implicit internal provider. If the implicit internal providers are though of
  as the root of a tree, then we're signaling a change in its subtree will
  trigger an update to anything that depends on it.
- For this parent internal provider, we need to find all
  AttributePrototypeArguments that are dependent to this provider. This will
  follow a similar path to starting at the top, resulting in a list of
  AttributeValues
- Now back to the set of AttributeValues (there will be several of them by this
  point). Each of these sets of values will depend on their common implicit
  internal provider which will have (for this set) an appropriate
  AttributeValue, meaning we end up with an AttrbiuteValue to AttributeValues
  dependency (in the topo/graph sense)
- Note we've only been talking about implicit internal providers so far, not
  external ones!
