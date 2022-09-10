use yew::Properties;
use yew_dom_attributes::{global_props::GlobalProps, li_props::LiProps};

#[derive(Debug, PartialEq, Clone)]
pub enum OtherProps {
    DivProps(GlobalProps),
    LiProps(LiProps),
}

#[derive(Debug, Properties, PartialEq, Clone)]
struct Props {
    #[prop_or_default]
    other_props: Option<OtherProps>,

    /// Flag to indicate if the DropDown menu is active or not (controlled).
    ///
    /// This API is generally used in conjunction with `closeOnClickOutside=true`
    /// since often we are controlling the active state by clicking another element
    /// within the document.
    #[prop_or_default]
    active: bool,
}

// 	/**
// 	 * Flag to align the DropDown menu within the viewport.
// 	 */
// 	alignmentByViewport?: React.ComponentProps<
// 		typeof Menu
// 	>['alignmentByViewport'];

// 	/**
// 	 * Default position of menu element. Values come from `./Menu`.
// 	 */
// 	alignmentPosition?: React.ComponentProps<typeof Menu>['alignmentPosition'];

// 	/**
// 	 * HTML element tag that the container should render.
// 	 */
// 	containerElement?: React.JSXElementConstructor<any> | 'div' | 'li';

// 	/**
// 	 * Flag that indicates whether to close DropDown when clicking on the item.
// 	 */
// 	closeOnClick?: boolean;

// 	closeOnClickOutside?: React.ComponentProps<
// 		typeof Menu
// 	>['closeOnClickOutside'];

// 	/**
// 	 *  Property to set the default value of `active` (uncontrolled).
// 	 */
// 	defaultActive?: boolean;

// 	/**
// 	 * Flag to indicate if menu contains icon symbols on the right side.
// 	 */
// 	hasRightSymbols?: boolean;

// 	/**
// 	 * Flag to indicate if menu contains icon symbols on the left side.
// 	 */
// 	hasLeftSymbols?: boolean;

// 	/**
// 	 * Prop to pass DOM element attributes to <DropDown.Menu />.
// 	 */
// 	menuElementAttrs?: React.HTMLAttributes<HTMLDivElement> &
// 		Pick<React.ComponentProps<typeof Menu>, 'containerProps'>;

// 	menuHeight?: React.ComponentProps<typeof Menu>['height'];

// 	menuWidth?: React.ComponentProps<typeof Menu>['width'];

// 	/**
// 	 * Callback for when the active state changes (controlled).
// 	 *
// 	 * This API is generally used in conjunction with `closeOnClickOutside=true`
// 	 * since often we are controlling the active state by clicking another element
// 	 * within the document.
// 	 */
// 	onActiveChange?: InternalDispatch<boolean>;

// 	/**
// 	 * Function for setting the offset of the menu from the trigger.
// 	 */
// 	offsetFn?: React.ComponentProps<typeof Menu>['offsetFn'];

// 	/**
// 	 * Flag indicating if the menu should be rendered lazily
// 	 */
// 	renderMenuOnClick?: boolean;

// 	/**
// 	 * Element that is used as the trigger which will activate the dropdown on click.
// 	 */
// 	trigger: React.ReactElement & {
// 		ref?: (node: HTMLButtonElement | null) => void;
// 	};
// }
